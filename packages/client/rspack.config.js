import path from 'node:path'
import fs from 'node:fs'
import url from 'node:url'
import rspack from '@rspack/core'
import HtmlWebpackPlugin from 'html-webpack-plugin'

const __dirname = path.dirname(url.fileURLToPath(import.meta.url))

const Args = {
  Production: 'production',
  Stats: '--stats',
}

const Paths = {
  Index: path.join(__dirname, 'src', 'index.ts'),
  Styles: path.join(__dirname, 'src', 'index.scss'),
  Output: path.join(__dirname, 'dist'),
}

const modes = {
  production: 'production',
  development: 'development',
}

let mode = modes.development

if (process.argv.includes(Args.Production)) {
  mode = modes.production
  process.env.NODE_ENV = modes.production
}

if (fs.existsSync(Paths.Output)) {
  fs.rmSync(Paths.Output, { recursive: true })
}

const config = {
  mode,
  devtool: 'source-map',
  entry:  {
    index: Paths.Index,
    styles: Paths.Styles,
  },
  output: {
    filename: '[name].[contenthash].js',
    path: Paths.Output,
  },
  experiments: {
    css: true,
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: [/node_modules/],
        loader: 'builtin:swc-loader',
        options: {
          jsc: {
            parser: {
              syntax: 'typescript',
            },
          },
        },
        type: 'javascript/auto',
      },
      {
        test: /\.(sass|scss)$/,
        use: [
          {
            loader: 'sass-loader',
            options: {
              // using `modern-compiler` and `sass-embedded` together significantly improve build performance,
              // requires `sass-loader >= 14.2.1`
              api: 'modern-compiler',
              implementation: await import('sass-embedded'),
            },
          },
        ],
        // set to 'css/auto' if you want to support '*.module.(scss|sass)' as CSS Modules, otherwise set type to 'css'
        type: 'css/auto',
      },
      {
        test: /\.css$/i,
        use: [rspack.CssExtractRspackPlugin.loader, 'css-loader'],
        type: 'javascript/auto',
      },
    ]
  },
  plugins: [
    new rspack.CssExtractRspackPlugin({
      filename: '[name].[chunkhash].css'
    }),
    new HtmlWebpackPlugin({
      minify: false,
      filename: 'index.html',
      template: 'src/index.html',
      inject:'head',
    }),
    {
      apply(compiler) {
        compiler.hooks.compilation.tap('ScriptAttributeInjector', (compilation) => 
          (HtmlWebpackPlugin).getHooks(compilation).alterAssetTags.tapAsync(
            'ScriptAttributeInjector', (data, cb) => {
              data.assetTags.scripts = data.assetTags.scripts.map(asset => {
                asset.attributes.type = 'module'
                return asset
              })
              return cb(null, data);
            }
          )
        )
      },
    },
    new rspack.CopyRspackPlugin({
      patterns: [
        { from: 'src/assets', to: 'assets' }
      ]
    }),
  ],
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    extensionAlias: {
      ".js": [".js", ".ts", ".tsx"],
    }
  },
  devServer: {
    hot: false,
    port: 8080,
    historyApiFallback: true,
    allowedHosts: 'all',
    host: '0.0.0.0',
    headers: [{
      key: 'Cache-Control',
      value: 'no-store'
    }],
    devMiddleware: {
      writeToDisk: true,
    }
  }
}

if (process.argv.includes(Args.Stats)) {
  config.plugins.push(new BundleAnalyzerPlugin())
}

export default config