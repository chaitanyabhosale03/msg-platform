import 'package:json_annotation/json_annotation.dart';

part 'protocol.g.dart';

@JsonSerializable()
class Message {
  final String id;
  final String type;
  final String from;
  final String? to;
  final Map<String, dynamic> payload;
  final int timestamp;
  final String? signature;

  Message({
    required this.id,
    required this.type,
    required this.from,
    this.to,
    required this.payload,
    required this.timestamp,
    this.signature,
  });

  factory Message.fromJson(Map<String, dynamic> json) => _$MessageFromJson(json);
  Map<String, dynamic> toJson() => _$MessageToJson(this);
}

@JsonSerializable()
class EncryptedMessage {
  final String ciphertext;
  final String nonce;
  final String ephemeralPublicKey;

  EncryptedMessage({
    required this.ciphertext,
    required this.nonce,
    required this.ephemeralPublicKey,
  });

  factory EncryptedMessage.fromJson(Map<String, dynamic> json) => _$EncryptedMessageFromJson(json);
  Map<String, dynamic> toJson() => _$EncryptedMessageToJson(this);
}
