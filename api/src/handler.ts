const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET,HEAD,POST,OPTIONS",
  "Access-Control-Max-Age": "86400",
}

async function handleRequest(request: Request): Promise<Response> {
  const headers = new Headers(corsHeaders)
  const response = new Response(JSON.stringify({msg:`Hello from Workers, [${request.url}]!`}), {
    headers: headers
  })
  return response
}

export {
  handleRequest
}