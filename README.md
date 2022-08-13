# RestSwiftly

This project draws inspiration from projects such as [Retrofit](https://square.github.io/retrofit/),
employing code generation for services.

## Introduction

A simple service can be defined as following:

```swift
protocol Return {
    // GET /get
    func get() async throws -> Hello
}
```

An instance can be obtained as following:

```swift
let service: Return = ReturnImpl(baseUrl: "http://httpbin.org")
```

Calls can then be awaited, or you can use a `Task` to generate a callback:

```swift
Task {
    let response = try await service.get()
    // do something with response, optionaly surround with do...try to handle errors
}
```

Samples of services can be found in [samples](samples) folder.

## Response

Responses will be decoded as json automatically, and need to conform to `Decodable`

## Request

A request body can optionally be posted on `PATCH`, `POST` or `PUT` methods, by naming the parameter
`body`. The parameter type needs to conform to `Encodable`.

## Request definition

The request will be defined by comments preceeding the function, general format is as follows:

```swift
// GET /path/{path}?q=:query
// Header-Name: header value
// Custom: {header}
func get(query: String, path: String, header: String)
```

The `GET /path?q=:query` defines a `GET` request to `/path/{path}` with placeholder, replaced with value from `path`,
with query parameter named `q`, whose value will be set to the value of parameter `query`. In a similar way, `Custom`
header will be set to value of `header` variable.

## Interceptors

Requests and responses can be intercepted by adding one or more `Interceptor`. Interceptors allow you to write code
common for multiple requests. For example:

```swift 
// Add a header to url paths that contain `auth`
class HeaderInterceptor: Interceptor {
    func intercept(chain: Chain, for request: URLRequest) async throws -> (Data, URLResponse) {
        let url = request.url!
        if url.absoluteString.contains("auth") {
            var request = request
            request.addValue("Bearer token", forHTTPHeaderField: "Authorization")
            return try await chain.proceed(with: request)
        }
        return try await chain.proceed(with: request)
    }
}
```

Or perhaps modifying/checking the response:

```swift
// Will print for each request returning 200
class ResponseInterceptor: Interceptor {
    func intercept(chain: Chain, for request: URLRequest) async throws -> (Data, URLResponse) {
        let (data, response) = try await chain.proceed(with: request)
        if let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 {
            print("Request was successful!")
        }
        return (data, response)
    }
}
```

## License

Project uses Apache 2.0 license. More info in [license file](LICENSE)