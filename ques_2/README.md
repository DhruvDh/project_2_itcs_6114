# ITCS 6114 - Speech to Text

## Installation
First, ensure you have [node.js](https://nodejs.org/en/) installed. Developed is done in v10.9, although there shouldn't be any issues if you run it on any other version.

Install the `yarn` package manager via `npm`. `npm` should be on your path if you correctly installed node.

```
npm install yarn
```

And then use yarn to install all node dependencies using - 

```
yarn install
```

## Usage

This software consists of two servers, namely `dashboard` and `jarvis`. `jarvis` handles all API calls to Google Cloud, while `dashboard` is used to serve the dashboard to the client. This is done for flexibility in terms of development and deployment technologies.

### dashboard

Use these commands to start the dashboard server -
```
cd dashboard
```

```
yarn serve
```

### jarvis

Use these commands to start the jarvis server -
```
cd jarvis
```

```
node index.js
```

> `jarvis` won't be able to successfully make API calls until you supply it with an Google cloud authentication key of your own.

## License

[MIT license](https://github.com/redkenrok/node-audiorecorder/blob/master/LICENSE)