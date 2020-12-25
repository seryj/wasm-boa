# Create deployment package:
mkdir -p deployment # this will be our deployment package
rm -r deployment/*  # remove old data
cp -r boa-web/pkg boa-web/static boa-web/bootstrap.js boa-web/favicon.ico boa-web/index.js deployment # copy required files