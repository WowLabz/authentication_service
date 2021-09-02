mongo <<EOF
use admin;
db.auth('root', 'dot_marketplace');
use authentication-service;
db.createUser({user: 'marketplace_admin', pwd: 'marketplace_pwd', roles: [{role: 'readWrite', db: 'authentication-service'}]});
EOF
