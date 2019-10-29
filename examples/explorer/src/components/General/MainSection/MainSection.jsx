import React from 'react';

import './mainSection.scss';

const MainSection = props => {
  const { children } = props;
  return (
    <div className="mainSection">
      <div className="mainCard">{children}</div>
    </div>
  );
};

export default MainSection;
