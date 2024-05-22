const log = require('loglevel');

log.setLevel('info');

if (process.env.NODE_ENV !== 'production') {
  log.setLevel('debug');
}

module.exports = log;