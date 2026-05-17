import 'package:sodium/sodium.dart';
import '../storage/encrypted_storage.dart';

class KeyManager {
  final EncryptedStorage _storage;
  late KeyPair _keyPair;
  late SigningKeyPair _signingKeyPair;

  KeyManager({required EncryptedStorage storage}) : _storage = storage;

  Future<void> initialize() async {
    // TODO: Load from encrypted storage or generate new
    // TODO: Store in encrypted local database
  }

  List<int> getPublicKey() => _keyPair.publicKey.extractBytes();
  List<int> getPrivateKey() => _keyPair.secretKey.extractBytes();
  List<int> getSigningPublicKey() => _signingKeyPair.publicKey.extractBytes();
  List<int> getSigningKey() => _signingKeyPair.secretKey.extractBytes();

  Future<void> generateKeys() async {
    // TODO: Generate X25519 and Ed25519 keypairs
    // TODO: Encrypt and store
  }

  Future<void> exportPublicKeys() async {
    // TODO: Export for identity registration
  }
}
