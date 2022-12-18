# Install package manager
sudo npm install -g yarn
# install typescript
sudo yarn add -g typescript
yarn link typescript
# Add package for generating react boilerplate
yarn add create-react-app
# generate typescript react boilerplate
yarn create-react-app frontend --template typescript
# remove create-react-app
rm -rf package.json node_modules yarn.lock
