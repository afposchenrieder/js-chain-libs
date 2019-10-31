import graphql from 'babel-plugin-relay/macro';

const epochQuery = graphql`
  query epochSearchResultQuery($id: EpochNumber!, $blocksCount: Int!) {
    epoch(id: $id) {
      ...FullEpochInfo_epoch @arguments(blocksCount: $blocksCount)
    }
  }
`;

export default epochQuery;
