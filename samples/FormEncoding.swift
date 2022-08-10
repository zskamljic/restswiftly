protocol FormEncoding {
	// POST /post
    // Content-Type: application/x-www-form-urlencoded
	func post(body: Hello) async throws
}
