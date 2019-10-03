import React from 'react';

import graphql from 'babel-plugin-relay/macro';
import { QueryRenderer } from 'react-relay';
import environment from '../../../graphql/environment';
import BlockInfo from '../../Blocks/BlockInfo/BlockInfo';
import Loading from '../../Commons/Loading/Loading';
import ErrorResult from '../../Commons/ErrorResult/ErrorResult';

import '../../generalStyling.scss';

const BlockSearchResult = ({ id }) => (
  <QueryRenderer
    className="queryResult"
    environment={environment}
    query={graphql`
      query BlockSearchResultQuery($id: String!) {
        block(id: $id) {
          id
          ...BlockInfo_block
        }
      }
    `}
    variables={{ id }}
    render={response => {
      const { error, props } = response;
      if (error) {
        return <ErrorResult />;
      }
      if (!props) {
        return <Loading />;
      }

      return <BlockInfo {...props} />;
    }}
  />
);

export default BlockSearchResult;
