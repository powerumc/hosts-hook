const dns = require('dns');
const http = require('http');

dns.lookup('example.com', (err, address, family) => {
  if (err) throw err;
  console.log('address:', address, 'family:', family);
});

http.get('http://example2.com', (res) => {
  console.log('STATUS:', res.statusCode);
}).on('error', (e) => {
  console.error(e.message);
});