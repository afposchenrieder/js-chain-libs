import winston from 'winston';

const { format } = winston;

const options = {
  console: {
    level: 'debug',
    timestamp: true,
    handleExceptions: true,
    json: false,
    format: format.combine(format.timestamp(), format.colorize(), format.simple())
  }
};

export default options;
