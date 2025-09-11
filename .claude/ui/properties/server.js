#!/usr/bin/env node

const http = require('http')
const fs = require('fs')
const path = require('path')

const PORT = 3000
const HOST = 'localhost'

// MIME 类型映射
const mimeTypes = {
  '.html': 'text/html',
  '.css': 'text/css',
  '.js': 'application/javascript',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.ico': 'image/x-icon',
}

function getMimeType(filePath) {
  const ext = path.extname(filePath).toLowerCase()
  return mimeTypes[ext] || 'text/plain'
}

// 创建服务器
const server = http.createServer((req, res) => {
  let pathname = req.url === '/' ? '/index.html' : req.url
  const filePath = path.join(__dirname, pathname)

  // 简单安全检查
  if (!filePath.startsWith(__dirname)) {
    res.writeHead(403)
    res.end('Forbidden')
    return
  }

  // 读取文件
  fs.readFile(filePath, (err, data) => {
    if (err) {
      res.writeHead(404)
      res.end('Not Found')
      return
    }

    const mimeType = getMimeType(filePath)
    res.writeHead(200, { 'Content-Type': mimeType })
    res.end(data)
  })
})

// 启动服务器
server.listen(PORT, HOST, () => {
  console.log(`🌐 服务器启动: http://${HOST}:${PORT}`)
  console.log('⌨️  按 Ctrl+C 停止')
})

// Ctrl+C 处理 - 直接强制退出
process.on('SIGINT', () => {
  console.log('\n🛑 关闭中...')
  process.exit(0)
})

// 错误处理
server.on('error', (err) => {
  console.error(`❌ 错误: ${err.message}`)
  process.exit(1)
})
