module.exports = {
  plugins: {
    'tailwindcss': {
      important: false,
      theme: {
        fontFamily: {
          'display': ['Oswald'],
          'body': ['Roboto Condensed'],
        },
        extend: {
          colors: {
            'mint-leaf': '#00b894',
            'light-greenish-blue': '#55efc4',
            'secondary': '#004C8F',
            'primary': '#3498db'
          }
        }
      },
    },
    // 'postcss-typography': {
    //   includeNormalize: false,
    // },
    'postcss-font-magician': {
      variants: {
        'Roboto Condensed': {
          '300': [],
          '400': [],
          '700': []
        },
        'Oswald': {
          '300': [],
          '400': [],
          '700': []
        }
      },
      foundries: ['google']
    },
    'autoprefixer': {},
  }
}