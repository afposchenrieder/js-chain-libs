import React from 'react';

import { QueryRenderer } from 'react-relay';
import environment from '../../../graphql/environment';
import { Loading, ErrorResult } from '../../Commons';
import logger from '../../../logger/logger';

const QueryWrapper = (WrappedComponent, query, propsConverter) => variables => {
  // Using no conversion if propsConverter is not defined
  const transform = propsConverter || (props => props);

  return (
    <QueryRenderer
      environment={environment}
      {...{ environment, query, variables }}
      render={response => {
        const { error, props } = response;
        if (error) {
          logger.error('There was an error fetching the information');
          return <ErrorResult />;
        }
        if (!props) {
          return <Loading />;
        }
        const finalProps = transform(props);
        return <WrappedComponent {...finalProps} />;
      }}
    />
  );
};

export default QueryWrapper;
