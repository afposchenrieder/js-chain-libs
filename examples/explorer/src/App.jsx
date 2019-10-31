import React from 'react';
import { Router } from '@reach/router';
import { MainSection, MainNavbar, SearchBar } from './components/General';
import { NotFound } from './components/Commons';
import { TABLE_PAGE_SIZE } from './helpers/constants';

import {
  RecentBlocks,
  TransactionSearchResult,
  BlockSearchResult,
  EpochSearchResult,
  BlockByLengthSearchResult,
  AddressSearchResult,
  StakePoolSearchResult
} from './components/MainSection';

import './generalStyling.scss';

const App = () => (
  <div className="app">
    <MainNavbar />
    {/* <StatusBar /> */}
    <SearchBar />
    <MainSection>
      <Router id="router">
        <NotFound default />
        <RecentBlocks blocksCount={TABLE_PAGE_SIZE} path="/" />
        <EpochSearchResult blocksCount={TABLE_PAGE_SIZE} path="epoch/:id" />
        <StakePoolSearchResult blocksCount={TABLE_PAGE_SIZE} path="pool/:id" />
        <AddressSearchResult txCount={TABLE_PAGE_SIZE} path="address/:bech32" />
        <TransactionSearchResult path="tx/:id" />
        <BlockSearchResult txCount={TABLE_PAGE_SIZE} path="block/:id" />
        <BlockByLengthSearchResult txCount={TABLE_PAGE_SIZE} path="block/chainLength/:length" />
      </Router>
    </MainSection>
  </div>
);

export default App;
