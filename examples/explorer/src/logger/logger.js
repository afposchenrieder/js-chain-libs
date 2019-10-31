import winston from 'winston';
import options from './loggerOptions';

const { format } = winston;

// TODO: Change this to a remote Transport to send
// logs to a log server
const logger = new winston.createLogger({
  transports: [new winston.transports.Console(options.console)],
  format: format.combine(
    format.timestamp({
      format: 'YYYY-MM-DD HH:mm:ss'
    }),
    format.json(),
    format.prettyPrint()
  ),
  exitOnError: false // do not exit on handled exceptions
});

export default logger;
