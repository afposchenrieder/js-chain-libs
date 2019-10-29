import React from 'react';
import { Router } from '@reach/router';
import { MainSection, MainNavbar, SearchBar } from './components/General';
import { NotFound } from './components/Commons';

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
        <RecentBlocks path="/" />
        <EpochSearchResult path="epoch/:id" />
        <StakePoolSearchResult path="pool/:id" />
        <AddressSearchResult path="address/:bech32" />
        <TransactionSearchResult path="tx/:id" />
        <BlockSearchResult path="block/:id" />
        <BlockByLengthSearchResult path="block/chainLength/:length" />
      </Router>
    </MainSection>
  </div>
);

export default App;
