import winston from 'winston';
import options from './loggerOptions';

const { format } = winston;

const logger = new winston.createLogger({
  transports: [new winston.transports.Console(options.console)],
  format: format.combine(
    format.timestamp({
      format: 'YYYY-MM-DD HH:mm:ss'
    }),
    format.prettyPrint()
  ),
  exitOnError: false // do not exit on handled exceptions
});

export default logger;
