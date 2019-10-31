import graphql from 'babel-plugin-relay/macro';

const addressQuery = graphql`
  query addressSearchQuery($bech32: String!, $txCount: Int) {
    address(bech32: $bech32) {
      ...FullAddressInfo_address @arguments(txCount: $txCount)
    }
  }
`;

export default addressQuery;
