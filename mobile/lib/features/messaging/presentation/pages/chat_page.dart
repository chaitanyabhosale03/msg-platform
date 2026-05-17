import 'package:flutter/material.dart';

class ChatPage extends StatefulWidget {
  const ChatPage({Key? key}) : super(key: key);

  @override
  State<ChatPage> createState() => _ChatPageState();
}

class _ChatPageState extends State<ChatPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Messages')),
      body: const Center(
        child: Text('TODO: Implement chat UI'),
      ),
      // TODO: Implement message list
      // TODO: Implement message input
      // TODO: Implement typing indicator
      // TODO: Implement read receipts
    );
  }
}
