# --- 1. Сборка Rust-генератора (capgen) ---
FROM rust:1-slim AS rustbuild
WORKDIR /gen
COPY generator/Cargo.toml ./Cargo.toml
COPY generator/src ./src
RUN cargo build --release

# --- 2. Сборка веб-приложения (.NET 10) ---
FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src
COPY AiCapcha.csproj ./
RUN dotnet restore
COPY Program.cs ./
COPY appsettings.json ./
COPY appsettings.Development.json ./
COPY Pages ./Pages
COPY wwwroot ./wwwroot
RUN dotnet publish -c Release -o /app

# --- 3. Runtime ---
FROM mcr.microsoft.com/dotnet/aspnet:10.0
WORKDIR /app
COPY --from=build /app ./
# кладём собранный генератор туда, где его ищет приложение (CapGen:Path)
COPY --from=rustbuild /gen/target/release/capgen /app/generator/target/release/capgen
ENV ASPNETCORE_URLS=http://0.0.0.0:7860
ENV ASPNETCORE_ENVIRONMENT=Production
EXPOSE 7860
ENTRYPOINT ["dotnet", "AiCapcha.dll"]