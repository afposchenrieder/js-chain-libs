import React from 'react';

import graphql from 'babel-plugin-relay/macro';
import { createRefetchContainer } from 'react-relay';
import logger from '../../../logger/logger';

import BlockTable from '../BlockTable/BlockTable';
import { getDescPageQuery, pageNumberDesc } from '../../../helpers/paginationHelper';

const RecentBlocksTable = ({ data, relay }) => {
  const connection = data.blocks;
  const currentPage = pageNumberDesc(connection);

  const handlePageChange = page => {
    const params = getDescPageQuery(page.current, connection.totalCount);
    const variables = {
      first: params.first || null,
      last: params.last || null,
      after: params.after || null,
      before: params.before || null
    };

    relay.refetch(variables, error => {
      if (error) {
        logger.error('There was an error refetching allBlocks ');
      }
    });
  };

  return (
    <div className="entityInfoContainer">
      <h2> Recent blocks </h2>
      <BlockTable {...{ currentPage, connection, handlePageChange }} />
    </div>
  );
};

export default createRefetchContainer(
  RecentBlocksTable,
  {
    data: graphql`
      fragment RecentBlocksTable_data on Query
        @argumentDefinitions(
          first: { type: "Int" }
          last: { type: "Int" }
          after: { type: "IndexCursor" }
          before: { type: "IndexCursor" }
        ) {
        blocks: allBlocks(first: $first, last: $last, after: $after, before: $before) {
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
    query RecentBlocksTableRefetchQuery(
      $first: Int
      $last: Int
      $after: IndexCursor
      $before: IndexCursor
    ) {
      ...RecentBlocksTable_data
        @arguments(first: $first, last: $last, after: $after, before: $before)
    }
  `
);
