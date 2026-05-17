import 'package:encrypted_shared_preferences/encrypted_shared_preferences.dart';

class EncryptedStorage {
  late EncryptedSharedPreferences _prefs;

  Future<void> initialize() async {
    _prefs = EncryptedSharedPreferences();
    // TODO: Initialize encryption keys from device keystore
  }

  Future<void> saveString(String key, String value) async {
    await _prefs.setString(key, value);
  }

  Future<String?> getString(String key) async {
    return _prefs.getString(key);
  }

  Future<void> saveBytes(String key, List<int> value) async {
    await _prefs.setString(key, String.fromCharCodes(value));
  }

  Future<List<int>?> getBytes(String key) async {
    final str = _prefs.getString(key);
    return str?.codeUnits;
  }

  Future<void> delete(String key) async {
    await _prefs.remove(key);
  }

  Future<void> clear() async {
    await _prefs.clear();
  }
}
