import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../crypto/crypto_service.dart';
import '../crypto/key_manager.dart';
import '../storage/encrypted_storage.dart';

final encryptedStorageProvider = Provider((ref) => EncryptedStorage());

final keyManagerProvider = Provider((ref) {
  final storage = ref.watch(encryptedStorageProvider);
  return KeyManager(storage: storage);
});

final cryptoServiceProvider = Provider((ref) {
  final keyManager = ref.watch(keyManagerProvider);
  return CryptoService(keyManager: keyManager);
});

final cryptoInitProvider = FutureProvider((ref) async {
  final storage = ref.watch(encryptedStorageProvider);
  final keyManager = ref.watch(keyManagerProvider);
  final crypto = ref.watch(cryptoServiceProvider);

  await storage.initialize();
  await keyManager.initialize();
  await crypto.initialize();
});
