import graphql from 'babel-plugin-relay/macro';

const stakePoolSearchQuery = graphql`
  query stakePoolSearchQuery($id: PoolId!, $blocksCount: Int) {
    stakePool(id: $id) {
      ...FullStakePoolInfo_stakePool @arguments(blocksCount: $blocksCount)
    }
  }
`;

export default stakePoolSearchQuery;
