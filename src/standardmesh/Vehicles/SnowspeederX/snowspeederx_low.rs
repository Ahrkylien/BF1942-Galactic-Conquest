subshader "snowspeeder_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	transparent true;
	envmap true;
	texture "texture/Vehicles/Snowspeeder/snowspeeder";
}

subshader "snowspeeder_lowX_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/undefined";
}

