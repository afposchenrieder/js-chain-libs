import graphql from 'babel-plugin-relay/macro';

// FIXME: the hardcoded 10 should be the
// value of TABLE_PAGE_SIZE constant
const allBlocksQuery = graphql`
  query allBlocksQuery($blocksCount: Int) {
    ...RecentBlocksTable_data @arguments(last: $blocksCount)
  }
`;
export default allBlocksQuery;
