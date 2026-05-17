import 'package:sodium/sodium.dart';
import 'key_manager.dart';

class CryptoService {
  late final Sodium _sodium;
  final KeyManager _keyManager;

  CryptoService({required KeyManager keyManager}) : _keyManager = keyManager;

  Future<void> initialize() async {
    _sodium = await Sodium.init();
  }

  /// Encrypt plaintext with recipient's public key (X25519)
  Future<List<int>> encryptTo(
    List<int> plaintext,
    List<int> recipientPublicKey,
  ) async {
    final privateKey = _keyManager.getPrivateKey();
    
    // TODO: Use box.easy for authenticated encryption
    // var ciphertext = _sodium.crypto.box.easy(
    //   message: plaintext,
    //   nonce: nonce,
    //   publicKey: recipientPublicKey,
    //   secretKey: privateKey,
    // );
    
    return [];
  }

  /// Decrypt ciphertext with sender's public key
  Future<List<int>> decryptFrom(
    List<int> ciphertext,
    List<int> senderPublicKey,
  ) async {
    final privateKey = _keyManager.getPrivateKey();
    
    // TODO: Implement decryption
    
    return [];
  }

  /// Sign message with private key (Ed25519)
  Future<List<int>> sign(List<int> message) async {
    final signKey = _keyManager.getSigningKey();
    
    // TODO: Implement signing
    // var signature = _sodium.crypto.sign.detached(
    //   message: message,
    //   secretKey: signKey,
    // );
    
    return [];
  }

  /// Verify message signature
  Future<bool> verify(
    List<int> message,
    List<int> signature,
    List<int> publicKey,
  ) async {
    // TODO: Implement verification
    return false;
  }
}
