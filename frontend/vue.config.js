require('dotenv').config({ path: require('find-config')('.env') })

module.exports = {
    devServer: {
      proxy: {
        '/api': {
          target: 'http://' + process.env.BACKEND_ADDRESS, // todo
          pathRewrite: {'^/api' : '/'}
        },
      }
    },
}