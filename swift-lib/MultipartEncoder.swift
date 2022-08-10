struct MultipartFile: Encodable {
    let name: String
    let contentType: String
    let data: Data

    init(with name: String, and data: Data) {
        self.name = name
        self.contentType = "application/octet-stream"
        self.data = data
    }

    init(with name: String, contentType: String, and data: Data) {
        self.name = name
        self.contentType = contentType
        self.data = data
    }
}

public class MultipartEncoder {
    let boundary: String

    init(boundary: String) {
        self.boundary = boundary
    }

    public func encode<T: Encodable>(_ value: T) throws -> Data {
        let multipartEncoding = MultipartEncoding(boundary: boundary)
        try value.encode(to: multipartEncoding)
        return multipartEncoding.data.multipartData()
    }
}

fileprivate class MultipartEncoding: Encoder {
    public var codingPath: [CodingKey] = []
    public var userInfo: [CodingUserInfoKey: Any] = [:]

    fileprivate final class BodyData {
        private var result = Data()
        private let boundary: String
        
        init(boundary: String) {
            self.boundary = boundary
        }

        func encode(key codingKey: [CodingKey], value: String) {
            let key = codingKey.map { $0.stringValue }.joined(separator: "")
            result.append("--\(boundary)\r\nContent-Disposition: form-data; name=\"\(key)\"\r\n\r\n".data(using: .utf8)!)
            result.append(value.data(using: .utf8)!)
            result.append("\r\n".data(using: .utf8)!)
        }

        func encode(key codingKey: [CodingKey], value: MultipartFile) {
            let key = codingKey.map { $0.stringValue }.joined(separator: "")
            result.append("--\(boundary)\r\nContent-Disposition: form-data; name=\"\(key)\"; filename=\"\(value.name)\"\r\nContent-Type: \(value.contentType)\r\n\r\n".data(using: .utf8)!)
            result.append(value.data)
            result.append("\r\n".data(using: .utf8)!)
        }

        func multipartData() -> Data {
            var data = result
            data.append("--\(boundary)--".data(using: .utf8)!)
            return data
        }
    }

    fileprivate var data: BodyData

    init(to data: BodyData) {
        self.data = data;
    }
    
    init(boundary: String) {
        self.data = BodyData(boundary: boundary)
    }

    public func container<Key: CodingKey>(keyedBy type: Key.Type) -> KeyedEncodingContainer<Key> {
        let container = MultipartEncodingContainer<Key>(to: data)
        return KeyedEncodingContainer(container)
    }

    public func singleValueContainer() -> SingleValueEncodingContainer {
        var container = MultipartSingleValueContainer(to: data)
        container.codingPath = codingPath
        return container
    }

    public func unkeyedContainer() -> UnkeyedEncodingContainer {
        var container = MultipartUnkeyedContainer(to: data)
        container.codingPath = codingPath
        return container
    }
}


fileprivate class MultipartEncodingContainer<Key: CodingKey>: KeyedEncodingContainerProtocol {
    private let data: MultipartEncoding.BodyData

    public var codingPath: [CodingKey] = []

    fileprivate init(to data: MultipartEncoding.BodyData) {
        self.data = data
    }

    func encode(_ value: Int16, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: UInt16, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: Float, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: UInt64, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: Bool, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: Int, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: String, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value)
    }

    func encode(_ value: Int64, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: UInt, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: UInt8, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode<T: Encodable>(_ value: T, forKey key: Key) throws {
        if value is MultipartFile {
            data.encode(key: codingPath + [key], value: value as! MultipartFile)
            return
        }

        let multipartEncoding = MultipartEncoding(to: data)
        multipartEncoding.codingPath.append(key)
        try value.encode(to: multipartEncoding)
    }

    func encode(_ value: UInt32, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: Double, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: Int32, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encode(_ value: Int8, forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: value.description)
    }

    func encodeNil(forKey key: Key) throws {
        data.encode(key: codingPath + [key], value: "")
    }

    func nestedContainer<NestedKey: CodingKey>(
        keyedBy keyType: NestedKey.Type,
        forKey key: Key
    ) -> KeyedEncodingContainer<NestedKey> {
        let container = MultipartEncodingContainer<NestedKey>(to: data)
        container.codingPath += [key]
        return KeyedEncodingContainer(container)
    }

    func nestedUnkeyedContainer(forKey key: Key) -> UnkeyedEncodingContainer {
        var container = MultipartUnkeyedContainer(to: data)
        container.codingPath += [key]
        return container
    }

    func superEncoder() -> Encoder {
        let superKey = Key(stringValue: "super")!
        return superEncoder(forKey: superKey)
    }

    func superEncoder(forKey key: Key) -> Encoder {
        let multipartEncoding = MultipartEncoding(to: data)
        multipartEncoding.codingPath += [key]
        return multipartEncoding
    }
}

fileprivate struct MultipartUnkeyedContainer: UnkeyedEncodingContainer {
    private let data: MultipartEncoding.BodyData

    var codingPath: [CodingKey] = []
    private(set) var count: Int = 0

    private struct IndexedCodingKey: CodingKey {
        let intValue: Int?
        let stringValue: String

        init(intValue: Int) {
            self.intValue = intValue
            self.stringValue = "[\(intValue)]"
        }

        init?(stringValue: String) {
            return nil
        }
    }

    init(to data: MultipartEncoding.BodyData) {
        self.data = data
    }

    private mutating func nextIndexedKey() -> CodingKey {
        let nextCodingKey = IndexedCodingKey(intValue: count)
        count += 1
        return nextCodingKey
    }

    mutating func encodeNil() throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: "")
    }

    mutating func encode(_ value: Float) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: Double) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: Bool) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: Int) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: UInt) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: UInt8) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: Int64) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: UInt16) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: String) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value)
    }

    mutating func encode(_ value: UInt32) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: Int8) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode(_ value: Int16) throws {
        data.encode(key: codingPath + [nextIndexedKey()], value: value.description)
    }

    mutating func encode<T: Encodable>(_ value: T) throws {
        if value is MultipartFile {
            data.encode(key: codingPath, value: value as! MultipartFile)
            return
        }

        let multipartEncoding = MultipartEncoding(to: data)
        multipartEncoding.codingPath.append(nextIndexedKey())
        try value.encode(to: multipartEncoding)
    }

    mutating func nestedContainer<NestedKey: CodingKey>(
        keyedBy keyType: NestedKey.Type
    ) -> KeyedEncodingContainer<NestedKey> {
        let container = MultipartEncodingContainer<NestedKey>(to: data)
        container.codingPath += [nextIndexedKey()]
        return KeyedEncodingContainer(container)
    }

    mutating func nestedUnkeyedContainer() -> UnkeyedEncodingContainer {
        var container = MultipartUnkeyedContainer(to: data)
        container.codingPath += [nextIndexedKey()]
        return container
    }

    mutating func superEncoder() -> Encoder {
        let multipartEncoding = MultipartEncoding(to: data)
        multipartEncoding.codingPath.append(nextIndexedKey())
        return multipartEncoding
    }
}

fileprivate struct MultipartSingleValueContainer: SingleValueEncodingContainer {
    private let data: MultipartEncoding.BodyData
    var codingPath: [CodingKey] = []

    init(to data: MultipartEncoding.BodyData) {
        self.data = data
    }

    func encodeNil() throws {
        data.encode(key: codingPath, value: "")
    }

    func encode(_ value: Bool) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: String) throws {
        data.encode(key: codingPath, value: value)
    }

    func encode(_ value: Double) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: Float) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: Int) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: Int8) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: Int16) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: Int32) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: Int64) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: UInt) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: UInt8) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: UInt16) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: UInt32) throws {
        data.encode(key: codingPath, value: value.description)
    }

    func encode(_ value: UInt64) throws {
        data.encode(key: codingPath, value: value.description)
    }
    
    func encode<T: Encodable>(_ value: T) throws {
        if value is MultipartFile {
            data.encode(key: codingPath, value: value as! MultipartFile)
            return
        }

        let multipartEncoding = MultipartEncoding(to: data)
        multipartEncoding.codingPath = codingPath
        try value.encode(to: multipartEncoding)
    }
}
