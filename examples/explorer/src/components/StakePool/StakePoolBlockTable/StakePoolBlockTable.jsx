import React from 'react';

import graphql from 'babel-plugin-relay/macro';
import { createRefetchContainer } from 'react-relay';
import logger from '../../../logger/logger';

import { BlockTable } from '../../Block';
import { pageNumberDesc, getDescPageQuery } from '../../../helpers/paginationHelper';

const StakePoolBlockTable = ({ stakePool, relay }) => {
  if (!stakePool.blocks) {
    return null;
  }

  const connection = stakePool.blocks;
  const currentPage = pageNumberDesc(connection);

  const handlePageChange = page => {
    const params = getDescPageQuery(page.current, connection.totalCount);
    const variables = {
      poolId: stakePool.id,
      first: params.first || null,
      last: params.last || null,
      after: params.after || null,
      before: params.before || null
    };

    relay.refetch(variables, error => {
      if (error) {
        logger.error('There was an error refetching Pool blocks');
      }
    });
  };

  return (
    <>
      <h2>Blocks</h2>
      <BlockTable {...{ currentPage, connection, handlePageChange }} />
    </>
  );
};

export default createRefetchContainer(
  StakePoolBlockTable,
  {
    stakePool: graphql`
      fragment StakePoolBlockTable_stakePool on Pool
        @argumentDefinitions(
          first: { type: "Int" }
          last: { type: "Int" }
          after: { type: "IndexCursor" }
          before: { type: "IndexCursor" }
        ) {
        id
        blocks(first: $first, last: $last, after: $after, before: $before) {
          edges {
            cursor
            node {
              id
              date {
                epoch {
                  id
                }
                slot
              }
              chainLength
              transactions {
                totalCount
              }
            }
          }
          pageInfo {
            hasNextPage
            hasPreviousPage
            endCursor
            startCursor
          }
          totalCount
        }
      }
    `
  },
  graphql`
    query StakePoolBlockTableRefetchQuery(
      $poolId: PoolId!
      $first: Int
      $last: Int
      $after: IndexCursor
      $before: IndexCursor
    ) {
      stakePool(id: $poolId) {
        ...StakePoolBlockTable_stakePool
          @arguments(first: $first, last: $last, after: $after, before: $before)
      }
    }
  `
);
