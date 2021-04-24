#!/usr/bin/env node
const { program } = require('commander')
const bs58 = require('bs58')
const crypto = require('crypto')
const fs = require('fs')
const path = require('path')
const tar = require('tar')
const realZlibConstants = require('zlib').constants

program.version(require('./package.json').version)

program
  .command('pack <path-to-project> <filename>')
  .description('pack the contract source code to the proper bs58-encoded .tar.gz archive')
  .action((sourceCodePath, filename) => {
    tar.c(
      {
        gzip: { level: realZlibConstants.Z_BEST_COMPRESSION, strategy: realZlibConstants.Z_DEFAULT_STRATEGY },
        file: path.resolve(filename + '.tar.gz'),
        C: path.normalize(sourceCodePath),
        filter: function (name) {
          const pathResolved = path.resolve(sourceCodePath, name)
          const isDir = fs.lstatSync(pathResolved).isDirectory()
          const base = path.basename(pathResolved)
          const ext = path.extname(pathResolved)
          const process = (isDir && base !== 'target' && base !== 'res') || base === 'Cargo.lock' || base === 'Cargo.toml' || ext === '.rs'
          if (process) {
            console.log(base, 'processing...')
          }
          return process
        }
      },
      // TODO replace with actual files to remove './' prefixes from tar
      ['.']
    ).then(_ => {
      const bytes = fs.readFileSync(path.resolve(filename + '.tar.gz'))
      // TODO use base64 encoding
      console.log('bs58 encoding...')
      const bs58encoded = bs58.encode(bytes)
      fs.writeFileSync(filename, bs58encoded)
      fs.unlinkSync(path.resolve(filename + '.tar.gz'))
      console.log('done')
    })
  })

program
  .command('unpack <filename>')
  .description('unpack the contract source code from the bs58-encoded .tar.gz archive')
  .action((filename) => {
    const bytes = fs.readFileSync(filename)
    console.log('bs58 decoding...')
    const bs58decoded = bs58.decode(bytes.toString())
    fs.writeFileSync(path.resolve(filename + '.tar.gz'), bs58decoded)
    console.log('files extracting...')
    tar.x(
      {
        file: path.resolve(filename + '.tar.gz')
      }
    ).then(_ => {
      fs.unlinkSync(path.resolve(filename + '.tar.gz'))
      console.log('done')
    })
  })

program
  .command('hash <wasm-file>')
  .description('get code_hash from the compiled wasm file')
  .action((wasmFile) => {
    const bytes = fs.readFileSync(wasmFile)
    const hash = crypto.createHash('sha256').update(bytes).digest('hex')
    const bs58encoded = bs58.encode(Buffer.from(hash, 'hex'))
    console.log(bs58encoded)
  })

;(async () => {
  await program.parseAsync(process.argv)
})()
