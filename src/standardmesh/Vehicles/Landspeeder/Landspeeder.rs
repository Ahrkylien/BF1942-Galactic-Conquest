subshader "Landspeeder_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.34902 0.34902 0.34902;
	lightingSpecular true;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 30.0;
	texture "texture/vehicles/landspeeder/SteeringWheel";
}
subshader "Landspeeder_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.505882 0.505882 0.505882;
	lightingSpecular true;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 50.0;
	texture "texture/vehicles/landspeeder/Throttle";
}
subshader "Landspeeder_Material2" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.498039 0.498039 0.498039;
	lightingSpecular true;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 30.0;
	transparent true;
	texture "texture/vehicles/landspeeder/Windshield";
}
subshader "Landspeeder_Material3" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.839216 0.360784 0.360784;
	lightingSpecular false;
	texture "texture/vehicles/landspeeder/Body";
}
