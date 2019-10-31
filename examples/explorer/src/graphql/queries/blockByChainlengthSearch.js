import graphql from 'babel-plugin-relay/macro';

const blockQuery = graphql`
  query blockByChainlengthSearchResultQuery($length: ChainLength!, $txCount: Int) {
    blockByChainLength(length: $length) {
      ...FullBlockInfo_block @arguments(txCount: $txCount)
    }
  }
`;

export default blockQuery;
