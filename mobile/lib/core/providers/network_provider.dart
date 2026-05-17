import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../network/websocket_client.dart';

final websocketClientProvider = Provider((ref) => WebSocketClient());

final websocketConnectionProvider = FutureProvider((ref) async {
  final client = ref.watch(websocketClientProvider);
  await client.connect();
  
  // TODO: Implement auto-reconnection
  // TODO: Implement message queue
  // TODO: Implement heartbeat
});
