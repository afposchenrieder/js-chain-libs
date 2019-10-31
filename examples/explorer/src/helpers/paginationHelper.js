import { TABLE_PAGE_SIZE } from './constants';

/**
 * Calculates the smaller page, could be the last or the first
 * depending if it's ordered descending or ascending
 */
const pageRemainder = (totalCount, pageSize) => {
  return totalCount % pageSize;
};

const pageOffset = (totalCount, pageSize) => {
  return pageSize - pageRemainder(totalCount, pageSize);
};

export const getNodeList = connection => {
  return connection.edges.map(edge => edge.node);
};

// Given a connection object, returns the query params for the next page
export const getNextPageQueryParam = connection => {
  return { after: connection.pageInfo.endCursor, first: TABLE_PAGE_SIZE };
};

// Given a connection object, returns the query params for the previous page
export const getPreviousPageQueryParam = connection => {
  return { before: connection.pageInfo.startCursor, last: TABLE_PAGE_SIZE };
};

/* This function calculates the page number based on the endCursor of the
 * Graphql Connection object. We rely on the cursor to be an increasing
 * and continous number starting with zero
 */
export const pageNumberDesc = (connection, pageSize = TABLE_PAGE_SIZE) => {
  const pageEnd = connection.pageInfo.endCursor;

  return Math.floor(Number.parseInt(pageEnd, 10) / pageSize) + 1;
};

/* This function calculates the query params
 * to use in order to get an specific page. We rely on the cursor
 * to be an increasing and continous number starting with zero
 */
export const getDescPageQuery = (pageNum, totalCount, pageSize = TABLE_PAGE_SIZE) => {
  const offset = pageOffset(totalCount, pageSize);
  const pageEnd = pageNum * pageSize + 1;

  return {
    before: (pageEnd - offset).toString(),
    last: pageSize
  };
};
