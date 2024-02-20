// swift-tools-version: 5.6
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "xosms_swift_bridge",
    products: [
        .library(name: "xosms_swift_bridge", type: .static, targets: ["xosms_swift_bridge"]),
    ],
    dependencies: [
    ],
    targets: [
        .target(
            name: "xosms_swift_bridge",
            dependencies: []
        )
    ]
)