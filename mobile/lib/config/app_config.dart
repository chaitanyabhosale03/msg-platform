class AppConfig {
  static const String appName = 'SecureMsg';
  static const String relayServerUrl = String.fromEnvironment(
    'RELAY_SERVER_URL',
    defaultValue: 'ws://localhost:8080/ws',
  );
  static const String apiBaseUrl = String.fromEnvironment(
    'API_BASE_URL',
    defaultValue: 'http://localhost:8080',
  );
  
  // Crypto constants
  static const int keySize = 32;
  static const int nonceSize = 12;
  static const int messageExpirationSecs = 2592000;
  
  // Timeouts
  static const Duration connectionTimeout = Duration(seconds: 30);
  static const Duration messageSyncTimeout = Duration(seconds: 5);
}
