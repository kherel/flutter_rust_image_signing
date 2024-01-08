import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_rust_image_signing/src/rust/api/simple.dart';
import 'package:flutter_rust_image_signing/src/rust/frb_generated.dart';
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  bool isSaved = false;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: ElevatedButton(
          onPressed: () async {
            final publicKey = await loadFromAssets('public_key.pem');
            final certificate = await loadFromAssets('certificate.pem');
            final image = await loadFromAssets('example.png');

            final signature = sign(
              certificateContents: image,
              privateKeyContents: publicKey,
              imageContents: certificate,
            );
            await saveUint8ListToFile(signature, 'signature.pem');
            setState(() {
              isSaved = true;
            });
          },
          child: Text(isSaved ? 'Signature saved' : 'Call Rust'),
        ),
      ),
    );
  }
}

Future<Uint8List> loadFromAssets(String filename) async {
  return (await rootBundle.load('assets/$filename')).buffer.asUint8List();
}

Future<String> saveUint8ListToFile(Uint8List bites, String fileName) async {
  final directory = await getApplicationDocumentsDirectory();
  final path = '${directory.path}/$fileName';
  final file = File(path);
  await file.writeAsBytes(bites);
  return path;
}
