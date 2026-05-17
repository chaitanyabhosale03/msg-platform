import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../storage/database.dart';

final databaseProvider = FutureProvider((ref) async {
  final db = AppDatabase();
  await db.initialize();
  return db;
});
