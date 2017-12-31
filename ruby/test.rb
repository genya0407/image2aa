require 'faraday'
require 'faraday_middleware'
require 'base64'

conn = Faraday.new "http://localhost:8000" do |conn|
  conn.request :multipart
  conn.request :url_encoded
  conn.response :json, :content_type => /\bjson$/

  conn.use :instrumentation
  conn.adapter :net_http
end

conn.post('/image', Faraday::UploadIO.new('./heidi.png', 'image/png'))