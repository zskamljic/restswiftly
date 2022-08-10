public class FormEncoder {
	public func encode<T: Encodable>(_ value: T) throws -> Data {
		let formEncoding = FormEncoding()
		try value.encode(to: formEncoding)
		return formEncoding.data.result.data(using: .utf8)
	}
}

fileprivate class FormEncoding: Encoder {
	public var codingPath: [CodingKey] = []
	public var userInfo: [CodingUserInfoKey: Any] = [:]

	fileprivate final class Data {
		private(set) var result = ""

		func encode(key codingKey: [CodingKey], value: String) {
			let key = codingKey.map { $0.stringValue }.joined(separator: "")
			if !result.isEmpty {
				result += "&"
			}
			result += "\(key)=\(value.addingPercentEncoding(withAllowedCharacters: .alphanumerics)!)"
		}
	}

	fileprivate var data: Data

	fileprivate init(to data: Data = Data()) {
		self.data = data
	}

	public func container<Key: CodingKey>(keyedBy type: Key.Type) -> KeyedEncodingContainer<Key> {
		let container = FormEncodingContainer<Key>(to: data)
		return KeyedEncodingContainer(container)
	}

	public func singleValueContainer() -> SingleValueEncodingContainer {
		var container = FormSingleValueContainer(to: data)
		container.codingPath = codingPath
		return container
	}

	public func unkeyedContainer() -> UnkeyedEncodingContainer {
		var container = FormUnkeyedContainer(to: data)
		container.codingPath = codingPath
		return container
	}
}

fileprivate class FormEncodingContainer<Key: CodingKey>: KeyedEncodingContainerProtocol {
	private let data: FormEncoding.Data

	public var codingPath: [CodingKey] = []

	fileprivate init(to data: FormEncoding.Data) {
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
		let formEncoding = FormEncoding()
		formEncoding.codingPath.append(key)
		try value.encode(to: formEncoding)
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
		let container = FormEncodingContainer<NestedKey>(to: data)
		container.codingPath += [key]
		return KeyedEncodingContainer(container)
	}

	func nestedUnkeyedContainer(forKey key: Key) -> UnkeyedEncodingContainer {
		var container = FormUnkeyedContainer(to: data)
		container.codingPath += [key]
		return container
	}

	func superEncoder() -> Encoder {
		let superKey = Key(stringValue: "super")!
		return superEncoder(forKey: superKey)
	}

	func superEncoder(forKey key: Key) -> Encoder {
		let formEncoding = FormEncoding(to: data)
		formEncoding.codingPath += [key]
		return formEncoding
	}
}

fileprivate struct FormUnkeyedContainer: UnkeyedEncodingContainer {
	private let data: FormEncoding.Data

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

	init(to data: FormEncoding.Data) {
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
		let formEncoding = FormEncoding(to: data)
		formEncoding.codingPath.append(nextIndexedKey())
		try value.encode(to: formEncoding)			
	}

	mutating func nestedContainer<NestedKey: CodingKey>(
		keyedBy keyType: NestedKey.Type
	) -> KeyedEncodingContainer<NestedKey> {
		let container = FormEncodingContainer<NestedKey>(to: data)
		container.codingPath += [nextIndexedKey()]
		return KeyedEncodingContainer(container)
	}

	mutating func nestedUnkeyedContainer() -> UnkeyedEncodingContainer {
        var container = FormUnkeyedContainer(to: data)
        container.codingPath += [nextIndexedKey()]
        return container
    }

	mutating func superEncoder() -> Encoder {
        let formEncoding = FormEncoding(to: data)
        formEncoding.codingPath.append(nextIndexedKey())
        return formEncoding
    }
}

fileprivate struct FormSingleValueContainer: SingleValueEncodingContainer {
	private let data: FormEncoding.Data
	var codingPath: [CodingKey] = []

	init(to data: FormEncoding.Data) {
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
        let formEncoding = FormEncoding(to: data)
        formEncoding.codingPath = codingPath
        try value.encode(to: formEncoding)
    }
}
