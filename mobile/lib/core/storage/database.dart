import 'package:sqflite/sqflite.dart';
import 'package:path/path.dart';

class AppDatabase {
  static const String _dbName = 'msg_platform.db';
  static const int _dbVersion = 1;

  late Database _database;

  Future<void> initialize() async {
    final path = join(await getDatabasesPath(), _dbName);
    _database = await openDatabase(
      path,
      version: _dbVersion,
      onCreate: _onCreate,
    );
  }

  Future<void> _onCreate(Database db, int version) async {
    // TODO: Create tables for messages, contacts, sessions
    // await db.execute(
    //   'CREATE TABLE messages ('
    //   'id TEXT PRIMARY KEY,'
    //   'from_id TEXT NOT NULL,'
    //   'to_id TEXT NOT NULL,'
    //   'ciphertext TEXT NOT NULL,'
    //   'timestamp INTEGER NOT NULL,'
    //   'is_read INTEGER DEFAULT 0,'
    //   'created_at INTEGER NOT NULL'
    //   ')'
    // );
  }

  Database get database => _database;

  Future<void> close() async {
    await _database.close();
  }
}
