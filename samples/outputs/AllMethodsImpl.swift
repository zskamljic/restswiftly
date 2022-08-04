class AllMethodsImpl: AllMethods {
    func delete() async throws {
        let url = URL("https://httpbin.org/delete")!
        var request = URLRequest(url: url)
        request.httpMethod = "DELETE"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        if let data = data {
            print(String(data: data, encoding: .utf8)!)
        }
    }

    func get() async throws {
        let url = URL("https://httpbin.org/get")!
        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        if let data = data {
            print(String(data: data, encoding: .utf8)!)
        }
    }

    func patch() async throws {
        let url = URL("https://httpbin.org/patch")!
        var request = URLRequest(url: url)
        request.httpMethod = "PATCH"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        if let data = data {
            print(String(data: data, encoding: .utf8)!)
        }
    }

    func post() async throws {
        let url = URL("https://httpbin.org/post")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        if let data = data {
            print(String(data: data, encoding: .utf8)!)
        }
    }

    func put() async throws {
        let url = URL("https://httpbin.org/put")!
        var request = URLRequest(url: url)
        request.httpMethod = "PUT"
        let (data, response) = try await URLSession.shared.data(for: request)
        guard (response as? HTTPURLResponse)?.statusCode == 200 else {
            fatalError("Unable to fetch data")
        }
        if let data = data {
            print(String(data: data, encoding: .utf8)!)
        }
    }

}
