import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';

void main() {
  // Подавляем debug ошибки для web
  if (kIsWeb) {
    WidgetsFlutterBinding.ensureInitialized();
  }
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(home: ProfileScreen(companyData: CompanyData.defaultData()));
  }
}

class ProfileScreen extends StatelessWidget {
  const ProfileScreen({super.key, required this.companyData});
  final CompanyData companyData;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: Size.fromHeight(200.0), // Динамическая высота
        child: AppBar(
          backgroundColor: Colors.transparent,
          elevation: 0,
          flexibleSpace: Container(
            decoration: BoxDecoration(
              image: DecorationImage(
                image: AssetImage(companyData.logo),
                fit: BoxFit.cover,
                onError: (error, stackTrace) {
                  // Обработка ошибки загрузки изображения
                },
              ),
            ),
            child: Container(
              decoration: BoxDecoration(
                gradient: LinearGradient(
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                  colors: [
                    Colors.black.withOpacity(0.3),
                    Colors.transparent,
                    Colors.black.withOpacity(0.7),
                  ],
                ),
              ),
              child: SafeArea(
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          IconButton(
                            icon: Icon(Icons.arrow_back, color: Colors.white),
                            onPressed: () => Navigator.of(context).pop(),
                          ),
                          IconButton(
                            icon: Icon(Icons.more_vert, color: Colors.white),
                            onPressed: () {},
                          ),
                        ],
                      ),
                      Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            'Profile',
                            style: TextStyle(
                              color: Colors.white,
                              fontSize: 24,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                          Text(
                            companyData.name,
                            style: TextStyle(
                              color: Colors.white70,
                              fontSize: 16,
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Компания добавлена в закладки!')),
          );
        },
        icon: Icon(Icons.bookmark),
        label: Text('Добавить в закладки'),
      ),
      floatingActionButtonLocation: FloatingActionButtonLocation.endTop,
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              companyData.name,
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            SizedBox(height: 8),
            Text(companyData.description),
            SizedBox(height: 16),
            ListTile(
              leading: Icon(Icons.web),
              title: Text('Website'),
              subtitle: Text(companyData.website),
            ),
            ListTile(
              leading: Icon(Icons.email),
              title: Text('Email'),
              subtitle: Text(companyData.email),
            ),
            ListTile(
              leading: Icon(Icons.phone),
              title: Text('Phone'),
              subtitle: Text(companyData.phone),
            ),
            ListTile(
              leading: Icon(Icons.location_on),
              title: Text('Address'),
              subtitle: Text('${companyData.address}, ${companyData.city}, ${companyData.state} ${companyData.zip}, ${companyData.country}'),
            ),
          ],
        ),
      ),
    );
  }
}

class CompanyData {
  final String name;
  final String logo;
  final String description;
  final String website;
  final String email;
  final String phone;
  final String address;
  final String city;
  final String state;
  final String zip;
  final String country;

  CompanyData({
    required this.name,
    required this.logo,
    required this.description,
    required this.website,
    required this.email,
    required this.phone,
    required this.address,
    required this.city,
    required this.state,
    required this.zip,
    required this.country,
  });

  CompanyData.defaultData()
    : name = 'Everest Clinic',
      logo =
          'assets/images/img.png',
      description = 'Everest Clinic is a clinic that provides medical services to the community.',
      website = 'https://everestclinic.com',
      email = 'info@everestclinic.com',
      phone = '1234567890',
      address = '123 Main St',
      city = 'New York',
      state = 'NY',
      zip = '10001',
      country = 'USA';
}
