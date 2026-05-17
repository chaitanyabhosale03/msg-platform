import 'package:sodium/sodium.dart';

class SessionManager {
  final Map<String, SessionState> _sessions = {};

  Future<void> initializeSession(String peerId, List<int> peerPublicKey) async {
    // TODO: Implement Signal Protocol Double Ratchet
    // TODO: Store ratchet state
    // TODO: Initialize with signed prekey
  }

  Future<List<int>> encryptMessage(String peerId, String message) async {
    // TODO: Use ratchet to encrypt
    return [];
  }

  Future<String> decryptMessage(String peerId, List<int> ciphertext) async {
    // TODO: Use ratchet to decrypt
    // TODO: Update ratchet state
    return '';
  }
}

class SessionState {
  final String peerId;
  List<int> rootKey = [];
  List<int> chainKeyRecv = [];
  List<int> chainKeySend = [];
  int messageNumber = 0;
  
  // TODO: Add prekey bundle state
  // TODO: Add device verification
  
  SessionState({required this.peerId});
}
