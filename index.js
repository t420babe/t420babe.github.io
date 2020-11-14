const rust = import('./pkg');

rust
  .then(m => m.greet('t420babe'))
  .catch(console.error);

