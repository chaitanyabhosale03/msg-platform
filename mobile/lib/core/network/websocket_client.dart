import 'package:web_socket_channel/web_socket_channel.dart';
import '../../../config/app_config.dart';
import 'protocol.dart';

class WebSocketClient {
  late WebSocketChannel _channel;
  final _messageController = <String, Function(Message)>{};

  Future<void> connect() async {
    try {
      _channel = WebSocketChannel.connect(
        Uri.parse(AppConfig.relayServerUrl),
      );
      
      // TODO: Send initial auth message
      // TODO: Start listening for messages
      // TODO: Implement reconnection logic
      
      _channel.stream.listen(
        (message) => _handleMessage(message),
        onError: (error) => _handleError(error),
        onDone: () => _handleClose(),
      );
    } catch (e) {
      // TODO: Handle connection errors
      rethrow;
    }
  }

  void send(Message message) {
    // TODO: Encrypt message before sending
    _channel.sink.add(message.toJson());
  }

  void subscribe(String messageType, Function(Message) handler) {
    _messageController[messageType] = handler;
  }

  void _handleMessage(dynamic data) {
    // TODO: Parse and decrypt message
    // TODO: Route to subscribers
  }

  void _handleError(dynamic error) {
    // TODO: Log and handle errors
  }

  void _handleClose() {
    // TODO: Attempt reconnection
    // TODO: Notify listeners
  }

  Future<void> disconnect() async {
    await _channel.sink.close();
  }
}
