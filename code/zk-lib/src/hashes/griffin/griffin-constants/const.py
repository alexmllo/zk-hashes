# Abrir el archivo original en modo lectura binaria
with open("griffin.bin", "rb") as original_file:
    # Leer las primeras 320 bytes (20 líneas * 16 bytes por línea)
    data = original_file.read(320)

# Abrir el archivo de destino en modo escritura binaria
with open("alphas2.bin", "wb") as target_file:
    # Escribir los datos leídos en el nuevo archivo
    target_file.write(data)
