import 'package:go_router/go_router.dart';
import '../features/messaging/presentation/pages/chat_page.dart';

final router = GoRouter(
  routes: [
    GoRoute(
      path: '/',
      builder: (context, state) => const ChatPage(),
      // TODO: Add routes for other pages
      // TODO: Add auth guards
      // TODO: Add deep linking
    ),
  ],
  // TODO: Add error handlers
  // TODO: Add redirect logic
);
