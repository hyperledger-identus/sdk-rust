// swift-tools-version: 6.0

import PackageDescription

let package = Package(
    name: "IdentusDid",
    platforms: [
        .iOS(.v15),
    ],
    products: [
        .library(name: "IdentusDid", targets: ["IdentusDid"]),
    ],
    targets: [
        .binaryTarget(
            name: "IdentusDidFFI",
            path: "Artifacts/IdentusDidFFI.xcframework"
        ),
        .target(
            name: "IdentusDid",
            dependencies: ["IdentusDidFFI"]
        ),
        .testTarget(
            name: "IdentusDidTests",
            dependencies: ["IdentusDid"]
        ),
    ]
)
