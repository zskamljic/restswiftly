protocol MultipartEncoding {
	// POST /post
    // Content-Type: multipart/form-data
	func post(body: Hello) async throws
}
