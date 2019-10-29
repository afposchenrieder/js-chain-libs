import { Environment, Network, RecordSource, Store } from 'relay-runtime';
import {
  RelayNetworkLayer,
  loggerMiddleware,
  cacheMiddleware,
  errorMiddleware,
  urlMiddleware,
  perfMiddleware
} from 'react-relay-network-modern';
import logger from '../logger/logger';
import configs from '../config.json';

const network = new RelayNetworkLayer([
  urlMiddleware({
    url: req => Promise.resolve(configs.explorerUrl)
  }),
  cacheMiddleware({
    size: 100, // max 100 requests
    ttl: 600000 // 1 minute
  }),
  perfMiddleware(logger.debug),
  loggerMiddleware(logger.verbose),
  errorMiddleware(logger.error)
]);

const environment = new Environment({
  network,
  store: new Store(new RecordSource())
});

export default environment;
